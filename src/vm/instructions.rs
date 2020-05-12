use std::collections::HashMap;

use crate::vm::VM;

macro_rules! opcodes {
    (enum $name:ident {
        $($variant:ident = $instruction:ident {
            byte: $byte:expr,
            info: $info:expr,
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
            
            pub fn info(&self) -> &'static str {
                match self {
                    $($name::$variant => $info,)*
                }
            }

            pub fn byte(&self) -> u8 {
                match self {
                    $($name::$variant => $byte,)*
                }
            }

            pub fn instruction(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($instruction),)*
                }
            }
            
            pub fn all() -> Vec<$name> {
                let mut list = vec![];
                $(list.push($name::$variant);)*
                list
            }
            
            pub fn map() -> HashMap<&'static str, $name> {
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

                $(if v_upper == stringify!($instruction).to_uppercase() { return $name::$variant; })*
                
                $(if v_upper == stringify!($variant).to_uppercase() { return $name::$variant; })*
                
                panic!("Unknown operator!")
            }
        }
    };
}

macro_rules! math_op {
    ($op:tt) => {
        &|vm: &mut VM| {
            let target = vm.read_u8() as usize;

            vm.registers[target]
                = vm.registers[vm.read_u8() as usize] $op vm.registers[vm.read_u8() as usize];

            true
        }
    };
}

macro_rules! math_f64_op {
    ($op:tt) => {
        &|vm: &mut VM| {
            let target = vm.read_u8() as usize;

            vm.float_registers[target]
                = vm.float_registers[vm.read_u8() as usize] $op vm.float_registers[vm.read_u8() as usize];

            true
        }
    };
}

macro_rules! condition_op {
    ($op:tt) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = vm.registers[vm.read_u8() as usize] $op vm.registers[vm.read_u8() as usize];

            true
        }
    };
}

macro_rules! condition_f64_op {
    (==) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = (vm.float_registers[vm.read_u8() as usize] - vm.float_registers[vm.read_u8() as usize]).abs() < f64::EPSILON;

            true
        }
    };
    (!=) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = (vm.float_registers[vm.read_u8() as usize] - vm.float_registers[vm.read_u8() as usize]).abs() > f64::EPSILON;

            true
        }
    };

    ($op:tt) => {
        &|vm: &mut VM| {
            vm.equal_flag
                = vm.float_registers[vm.read_u8() as usize] $op vm.float_registers[vm.read_u8() as usize];

            true
        }
    };
}

opcodes! {
    enum Opcode {
        Halt = HLT {
            byte: 0x00,
            info: "Stop execution immediately.",
            |_: &mut VM| false
        },
    
        // Integer register operations
        Set = SET {
            byte: 0x01, // <$target> <byte 1> <byte 2>
            info: "Set $target using constant bytes.",
            |vm: &mut VM| {
                let register = vm.read_u8() as usize;

                vm.registers[register] = i32::from(vm.read_u16());

                true
            }
        },
        Load = LOAD {
            byte: 0x02, // <$target> <$value>
            info: "Set $target to $value.",
            |vm: &mut VM| {
                let register = vm.read_u8() as usize;
                let pointer = vm.registers[vm.read_u8() as usize] as usize;

                vm.registers[register] = vm.fetch_heap_u32(pointer) as i32;

                true
            }
        },
        Store = STOR {
            byte: 0x03, // <#target> <$value>
            info: "Set #target to $value.",
            |vm: &mut VM| {
                let pointer = vm.read_u8() as usize;
                let register = vm.read_u8() as usize;

                vm.set_heap_u32(pointer, vm.registers[register] as u32);

                true
            }
        },
        Move = MOV {
            byte: 0x04, // <$target> <$value>
            info: "Set $target to $value.",
            |vm: &mut VM| {
                let target = vm.read_u8() as usize;

                vm.registers[target] = vm.registers[vm.read_u8() as usize];

                true
            }
        },
    
        Add = ADD {
            byte: 0x10, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 + $value2.",
            math_op!(+)
        },
        Subtract = SUB {
            byte: 0x11, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 - $value2.",
            math_op!(-)
        },
        Multiply = MUL {
            byte: 0x12, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 * $value2.",
            math_op!(*)
        },
        Divide = DIV {
            byte: 0x13, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 / $value2. Remainder in a dedicated register.",
            |vm: &mut VM| {
                let target = vm.read_u8() as usize;
                let register1 = vm.registers[vm.read_u8() as usize];
                let register2 = vm.registers[vm.read_u8() as usize];

                vm.registers[target] = register1 / register2;
                vm.remainder = (register1 % register2) as u32;

                true
            }
        },
    
        And = AND {
            byte: 0x14, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 & $value2.",
            math_op!(&)
        },
        Or = OR {
            byte: 0x15, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 | $value2.",
            math_op!(|)
        },
        XOR = XOR {
            byte: 0x16, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 ^ $value2.",
            math_op!(^)
        },
        ShiftLeft = SHL {
            byte: 0x17, // <$target> <$count>
            info: "Bit shift $target $count left.",
            |vm: &mut VM| {
                let register = vm.read_u8() as usize;
                let num_bits = vm.read_u8();

                vm.registers[register] = vm.registers[register].wrapping_shl(num_bits.into());

                true
            }
        },
        ShiftRight = SHR {
            byte: 0x18, // <$target> <$count>
            info: "Bit shift $target $count right.",
            |vm: &mut VM| {
                let register = vm.read_u8() as usize;
                let num_bits = vm.read_u8();

                vm.registers[register] = vm.registers[register].wrapping_shr(num_bits.into());

                true
            }
        },
        
        Increment = INC {
            byte: 0x19, // <$target>
            info: "Increment $target by 1.",
            |vm: &mut VM| {
                vm.registers[vm.read_u8() as usize] += 1;

                true
            }
        },
        Decrement = DEC {
            byte: 0x1A, // <$target>,
            info: "Decrement $target by 1.",
            |vm: &mut VM| {
                vm.registers[vm.read_u8() as usize] -= 1;

                true
            }
        },
    
        Equal = EQ {
            byte: 0x20, // <$value1> <$value2>
            info: "Sets Z flag if $value1 == $value2.",
            condition_op!(==)
        },
        NotEqual = NEQ {
            byte: 0x21, // <$value1> <$value2>
            info: "Sets Z flag if $value1 != $value2.",
            condition_op!(!=)
        },
        GreaterThan = GT {
            byte: 0x22, // <$value1> <$value2>
            info: "Sets Z flag if $value1 > $value2.",
            condition_op!(>)
        },
        LessThan = LT {
            byte: 0x23, // <$value1> <$value2>
            info: "Sets Z flag if $value1 < $value2.",
            condition_op!(<)
        },
        GreaterThanOrEqual = GEQ {
            byte: 0x24, // <$value1> <$value2>
            info: "Sets Z flag if $value1 >= $value2.",
            condition_op!(>=)
        },
        LessThanOrEqual = LEQ {
            byte: 0x25, // <$value> <$value>
            info: "Sets Z flag if $value1 <= $value2.",
            condition_op!(<=)
        },
        
        // Floating point register operations
        SetF64 = SETF {
            byte: 0x30, // <$target> <byte 1> <byte 2>
            info: "Set $target using constant bytes.",
            |vm: &mut VM| {
                let register = vm.read_u8() as usize;

                vm.float_registers[register] = f64::from(vm.read_u16());

                true
            }
        },
        LoadF64 = LOADF {
            byte: 0x31, // <$target> <#value>
            info: "Set $target to #value.",
            |vm: &mut VM| {
                let register = vm.read_u8() as usize;
                let pointer = vm.registers[vm.read_u8() as usize] as usize;

                vm.float_registers[register] = vm.fetch_heap_u64(pointer) as f64;

                true
            }
        },
        StoreF64 = STORF {
            byte: 0x32, // <#target> <$value>
            info: "Set #target to $value.",
            |vm: &mut VM| {
                let pointer = vm.read_u8() as usize;
                let register = vm.read_u8() as usize;

                vm.set_heap_u64(pointer, vm.float_registers[register] as u64);

                true
            }
        },
        MoveF64 = MOVF {
            byte: 0x33, // <$target> <$value>
            info: "Set $target to $value.",
            |vm: &mut VM| {
                vm.float_registers[vm.read_u8() as usize] = vm.float_registers[vm.read_u8() as usize];

                true
            }
        },

        AddF64 = ADDF {
            byte: 0x41, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 + $value2.",
            math_f64_op!(+)
        },
        SubtractF64 = SUBF {
            byte: 0x42, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 - $value2.",
            math_f64_op!(-)
        },
        MultiplyF64 = MULF {
            byte: 0x43, // <$target> <$value1> <$value2>
            info: "Set $target to $value1 * $value2.",
            math_f64_op!(*)
        },
        DivideF64 = DIVF {
            byte: 0x44, // <$target> <$value> <$value>
            info: "Set $target to $value1 / $value2.",
            math_f64_op!(/)
        },

        EqualF64 = EQF {
            byte: 0x51, // <$value> <$value>
            info: "Sets Z flag if $value1 == $value2.",
            condition_f64_op!(==)
        },
        NotEqualF64 = NEQF {
            byte: 0x52, // <$value> <$value>
            info: "Sets Z flag if $value1 != $value2.",
            condition_f64_op!(!=)
        },
        GreaterThanF64 = GTF {
            byte: 0x53, // <$value> <$value>
            info: "Sets Z flag if $value1 > $value2.",
            condition_f64_op!(>)
        },
        LessThanF64 = LTF {
            byte: 0x54, // <$value> <$value>
            info: "Sets Z flag if $value1 < $value2.",
            condition_f64_op!(<)
        },
        GreaterThanOrEqualF64 = GEQF {
            byte: 0x55, // <$value> <$value>
            info: "Sets Z flag if $value1 >= $value2.",
            condition_f64_op!(>=)
        },
        LessThanOrEqualF64 = LEQF {
            byte: 0x56, // <$value> <$value>
            info: "Sets Z flag if $value1 <= $value2.",
            condition_f64_op!(<=)
        },

        Jump = JMP {
            byte: 0x60, // <#byte>
            info: "Jump to #byte.",
            |vm: &mut VM| {
                vm.pc = vm.registers[vm.read_u8() as usize] as usize;

                true
            }
        },
        JumpForward = JMPF {
            byte: 0x61, // <$bytes>
            info: "Jump forward $bytes.",
            |vm: &mut VM| {
                let value = vm.registers[vm.read_u8() as usize];
                vm.pc += value as usize;

                true
            }
        },
        JumpBackward = JMPB {
            byte: 0x62, // <$bytes>
            info: "Jump backward $bytes.",
            |vm: &mut VM| {
                let value = vm.registers[vm.read_u8() as usize];
                vm.pc -= value as usize;

                true
            }
        },
        JumpIfEqual = JEQ {
            byte: 0x63, // <$byte>
            info: "If the Z flag is set, jump to $bytes.",
            |vm: &mut VM| {
                if vm.equal_flag {
                    let register = vm.read_u8() as usize;
    
                    vm.pc = vm.registers[register] as usize;
                } else {
                    // Skip the bits we ignored since equality is false
                    vm.pc += 1;
                }

                true
            }
        },
    }
}