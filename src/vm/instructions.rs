use std::collections::HashMap;

use crate::vm::VM;

macro_rules! opcodes {
    (enum $name:ident {
        $($variant:ident = {
            $byte:expr,
            $func:expr
        }),*,
    }) => {
        #[derive(Debug, PartialEq)]
        pub enum $name {
            $($variant = $byte,)*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant),)*
                }
            }

            pub fn byte(&self) -> u8 {
                match self {
                    $($name::$variant => $byte,)*
                }
            }
            
            pub fn all() -> HashMap<&'static str, $name> {
                let mut map = HashMap::new();
                $(map.insert(stringify!($variant), $name::$variant);)*
                $(map.insert(stringify!($variant), $name::$variant);)*
                map
            }

            pub fn call(vm: &mut VM, v: u8) -> bool {
                return match v {
                    $($byte => $func(vm),)*

                    _ => panic!("Unknown operator!"),
                }
            }
        }

        impl From<u8> for $name {
            fn from(v: u8) -> Self {
                return match v {
                    $($byte => $name::$variant,)*

                    _ => panic!("Unknown operator!"),
                }
            }
        }

        impl From<String> for $name {
            fn from(v: String) -> Self {
                let v_upper = v.to_uppercase();

                $(if v_upper == stringify!($variant).to_uppercase() { return $name::$variant; })*
                
                panic!("Unknown operator!")
            }
        }
    };
}

macro_rules! math_op {
    ($op:tt) => {
        &|vm: &mut VM| {
            // The next_8_bits() to the left of the equals is done following the ones after
            vm.registers[vm.next_8_bits() as usize]
                = vm.registers[vm.next_8_bits() as usize] $op vm.registers[vm.next_8_bits() as usize];

            true
        }
    };
}

macro_rules! math_f64_op {
    ($op:tt) => {
        &|vm: &mut VM| {
            // The next_8_bits() to the left of the equals is done following the ones after
            vm.float_registers[vm.next_8_bits() as usize]
                = vm.float_registers[vm.next_8_bits() as usize] $op vm.float_registers[vm.next_8_bits() as usize];

            true
        }
    };
}

macro_rules! condition_op {
    ($op:tt) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = vm.registers[vm.next_8_bits() as usize] $op vm.registers[vm.next_8_bits() as usize];

            true
        }
    };
}

macro_rules! condition_f64_op {
    (==) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = (vm.float_registers[vm.next_8_bits() as usize] - vm.float_registers[vm.next_8_bits() as usize]).abs() < f64::EPSILON;

            true
        }
    };
    (!=) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = (vm.float_registers[vm.next_8_bits() as usize] - vm.float_registers[vm.next_8_bits() as usize]).abs() > f64::EPSILON;

            true
        }
    };

    ($op:tt) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = vm.float_registers[vm.next_8_bits() as usize] $op vm.float_registers[vm.next_8_bits() as usize];

            true
        }
    };
}

opcodes! {
    enum Opcode {
        Halt = {
            0x00,
            |_: &mut VM| false
        },
    
        // Integer register operations
        Load = {
            0x01, // <$target> <int byte 1> <int byte 2>
            |vm: &mut VM| {
                let register = vm.next_8_bits() as usize;

                vm.registers[register] = i32::from(vm.next_16_bits());

                true
            }
        },
    
        Add = {
            0x02, // <$value> <$value> <$target>
            math_op!(+)
        },
        Subtract = {
            0x03, // <$value> <$value> <$target>
            math_op!(-)
        },
        Multiply = {
            0x04, // <$value> <$value> <$target>
            math_op!(*)
        },
        Divide = {
            0x05, // <$value> <$value> <$target>
            |vm: &mut VM| {
                let register1 = vm.registers[vm.next_8_bits() as usize];
                let register2 = vm.registers[vm.next_8_bits() as usize];

                vm.registers[vm.next_8_bits() as usize] = register1 / register2;
                vm.remainder = (register1 % register2) as u32;

                true
            }
        },
    
        ShiftLeft = {
            0x06, // <$target> <$count>
            |vm: &mut VM| {
                let register = vm.next_8_bits() as usize;
                let num_bits = match vm.next_8_bits() {
                    0 => { 16 },
                    other => { other }
                };

                vm.registers[register] = vm.registers[register].wrapping_shl(num_bits.into());

                true
            }
        },
        ShiftRight = {
            0x07, // <$target> <$count>
            |vm: &mut VM| {
                let register = vm.next_8_bits() as usize;
                
                let num_bits = match vm.next_8_bits() {
                    0 => { 16 },
                    other => { other }
                };

                vm.registers[register] = vm.registers[register].wrapping_shr(num_bits.into());

                true
            }
        },
        
        Increment = {
            0x08, // <$target>
            |vm: &mut VM| {
                vm.registers[vm.next_8_bits() as usize] += 1;

                true
            }
        },
        Decrement = {
            0x09, // <$target>,
            |vm: &mut VM| {
                vm.registers[vm.next_8_bits() as usize] -= 1;

                true
            }
        },
    
        Equal = {
            0x0A, // <$value> <$value>
            condition_op!(==)
        },
        NotEqual = {
            0x0B, // <$value> <$value>
            condition_op!(!=)
        },
        GreaterThan = {
            0x0C, // <$value> <$value>
            condition_op!(>)
        },
        LessThan = {
            0x0D, // <$value> <$value>
            condition_op!(<)
        },
        GreaterThanOrEqual = {
            0x0E, // <$value> <$value>
            condition_op!(>=)
        },
        LessThanOrEqual = {
            0x0F, // <$value> <$value>
            condition_op!(<=)
        },
        
        // Floating point register operations
        LoadF64 = {
            0x11, // <$target> <int byte 1> <int byte 2>
            |vm: &mut VM| {
                let register = vm.next_8_bits() as usize;

                vm.float_registers[register] = f64::from(vm.next_16_bits());

                true
            }
        },

        AddF64 = {
            0x12, // <$value> <$value> <$target>
            math_f64_op!(+)
        },
        SubtractF64 = {
            0x13, // <$value> <$value> <$target>
            math_f64_op!(-)
        },
        MultiplyF64 = {
            0x14, // <$value> <$value> <$target>
            math_f64_op!(*)
        },
        DivideF64 = {
            0x15, // <$value> <$value> <$target>
            math_f64_op!(/)
        },

        EqualF64 = {
            0x16, // <$value> <$value>
            condition_f64_op!(==)
        },
        NotEqualF64 = {
            0x17, // <$value> <$value>
            condition_f64_op!(!=)
        },
        GreaterThanF64 = {
            0x18, // <$value> <$value>
            condition_f64_op!(>)
        },
        LessThanF64 = {
            0x19, // <$value> <$value>
            condition_f64_op!(<)
        },
        GreaterThanOrEqualF64 = {
            0x1A, // <$value> <$value>
            condition_f64_op!(>=)
        },
        LessThanOrEqualF64 = {
            0x1B, // <$value> <$value>
            condition_f64_op!(<=)
        },

        Jump = {
            0x20, // <$byte>
            |vm: &mut VM| {
                let target = vm.registers[vm.next_8_bits() as usize];
                vm.pc = target as usize;

                true
            }
        },
        JumpForward = {
            0x21, // <$bytes>
            |vm: &mut VM| {
                let value = vm.registers[vm.next_8_bits() as usize];
                vm.pc += value as usize;

                true
            }
        },
        JumpBackward = {
            0x22, // <$bytes>
            |vm: &mut VM| {
                let value = vm.registers[vm.next_8_bits() as usize];
                vm.pc -= value as usize;

                true
            }
        },
        JumpIfEqual = {
            0x23, // <$byte>
            |vm: &mut VM| {
                if vm.equal_flag {
                    let register = vm.next_8_bits() as usize;
    
                    vm.pc = vm.registers[register] as usize;
                } else {
                    // Skip the bits we ignored since equality is false
                    vm.pc += 1;
                }

                true
            }
        },
    
        Move = {
            0x30, // <$value> <$target>
            |vm: &mut VM| {
                vm.registers[vm.next_8_bits() as usize] = vm.registers[vm.next_8_bits() as usize];

                true
            }
        },
    }
}