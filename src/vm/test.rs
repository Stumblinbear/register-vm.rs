#[cfg(test)]
mod tests {
    use crate::vm::instructions::Opcode;
    use crate::vm::VM;

    fn get_test_vm() -> VM {
        return VM::default();
    }

    #[test]
    fn create_vm() {
        let test_vm = get_test_vm();
        
        assert_eq!(test_vm.registers[0], 0)
    }

    macro_rules! math_op_test {
        ($opcode:expr, $val1:expr, $val2:expr, $result:expr) => {
            let mut test_vm = get_test_vm();
    
            test_vm.registers[0] = $val1;
            test_vm.registers[1] = $val2;
            test_vm.program = vec![$opcode, 0, 1, 2];
            test_vm.run_once();
    
            assert_eq!(test_vm.registers[2], $result);
        };
    }

    macro_rules! condition_op_test {
        ($opcode:expr, $turuthyReg1:expr, $turuthyReg2:expr, $falsyReg1:expr, $falsyReg2:expr) => {
            let mut test_vm = get_test_vm();
    
            test_vm.registers[0] = $turuthyReg1;
            test_vm.registers[1] = $turuthyReg2;
            test_vm.program = vec![$opcode, 0, 1, $opcode, 0, 1];
            test_vm.run_once();
    
            assert_eq!(test_vm.equal_flag, true);
    
            test_vm.registers[0] = $falsyReg1;
            test_vm.registers[1] = $falsyReg2;
            test_vm.run_once();
    
            assert_eq!(test_vm.equal_flag, false);
        };
    }

    macro_rules! math_f64_op_test {
        ($opcode:expr, $val1:expr, $val2:expr, $result:expr) => {
            let mut test_vm = get_test_vm();
    
            test_vm.float_registers[0] = $val1;
            test_vm.float_registers[1] = $val2;
            test_vm.program = vec![$opcode, 0, 1, 2];
            test_vm.run_once();
    
            assert_eq!(test_vm.float_registers[2], $result);
        };
    }

    macro_rules! condition_f64_op_test {
        ($opcode:expr, $turuthyReg1:expr, $turuthyReg2:expr, $falsyReg1:expr, $falsyReg2:expr) => {
            let mut test_vm = get_test_vm();
    
            test_vm.float_registers[0] = $turuthyReg1;
            test_vm.float_registers[1] = $turuthyReg2;
            test_vm.program = vec![$opcode, 0, 1, $opcode, 0, 1];
            test_vm.run_once();
    
            assert_eq!(test_vm.equal_flag, true);
    
            test_vm.float_registers[0] = $falsyReg1;
            test_vm.float_registers[1] = $falsyReg2;
            test_vm.run_once();
    
            assert_eq!(test_vm.equal_flag, false);
        };
    }


    #[test]
    fn halt_opcode() {
      let mut test_vm = get_test_vm();
      
      test_vm.program = vec![Opcode::Halt.byte()];
      test_vm.run();

      assert_eq!(test_vm.pc, 1);
    }


    #[test]
    fn load_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![Opcode::Load as u8, 0, 1, 244]; // 500 using two u8s in little endian
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn add_opcode() {
        math_op_test!(Opcode::Add.byte(),
            1, 2, 3
        );
    }

    #[test]
    fn subtract_opcode() {
        math_op_test!(Opcode::Subtract.byte(),
            5, 1, 4
        );
    }

    #[test]
    fn multiply_opcode() {
        math_op_test!(Opcode::Multiply.byte(),
            5, 2, 10
        );
    }

    #[test]
    fn divide_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 9;
        test_vm.registers[1] = 4;
        test_vm.program = vec![Opcode::Divide.byte(), 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 1);
    }


    #[test]
    fn shift_left_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 5;
        test_vm.program = vec![Opcode::ShiftLeft.byte(), 0, 0, Opcode::ShiftLeft.byte(), 0, 1];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 327680);
        
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 655360);
    }

    #[test]
    fn shift_right_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 655360;
        test_vm.program = vec![Opcode::ShiftRight.byte(), 0, 1, Opcode::ShiftRight.byte(), 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 327680);
        
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 5);
    }


    #[test]
    fn increment_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 0;
        test_vm.program = vec![Opcode::Increment.byte(), 0, Opcode::Increment.byte(), 0];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 2);
    }

    #[test]
    fn decrement_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 2;
        test_vm.program = vec![Opcode::Decrement.byte(), 0, Opcode::Decrement.byte(), 0];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 0);
    }


    #[test]
    fn equals_opcode() {
        condition_op_test!(Opcode::Equal.byte(),
            10, 10,
            10, 20
        );
    }

    #[test]
    fn not_equals_opcode() {
        condition_op_test!(Opcode::NotEqual.byte(),
            10, 20,
            10, 10
        );
    }

    #[test]
    fn greater_than_opcode() {
        condition_op_test!(Opcode::GreaterThan.byte(),
            10, 5,
            10, 10
        );
    }

    #[test]
    fn less_than_opcode() {
        condition_op_test!(Opcode::LessThan.byte(),
            5, 10,
            10, 5
        );
    }

    #[test]
    fn greater_than_or_equal_opcode() {
        condition_op_test!(Opcode::GreaterThanOrEqual.byte(),
            10, 10,
            5, 10
        );
    }

    #[test]
    fn less_than_or_equal_opcode() {
        condition_op_test!(Opcode::LessThanOrEqual.byte(),
            10, 10,
            10, 5
        );
    }


    #[test]
    fn f64_load_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![Opcode::LoadF64.byte(), 0, 1, 244]; // 500 using two u8s in little endian
        test_vm.run_once();

        assert_eq!(test_vm.float_registers[0], 500.0);
    }

    #[test]
    fn f64_add_opcode() {
        math_f64_op_test!(Opcode::AddF64.byte(),
            1.0, 2.0, 3.0
        );
    }

    #[test]
    fn f64_subtract_opcode() {
        math_f64_op_test!(Opcode::SubtractF64.byte(),
            5.0, 1.0, 4.0
        );
    }

    #[test]
    fn f64_multiply_opcode() {
        math_f64_op_test!(Opcode::MultiplyF64.byte(),
            5.0, 2.0, 10.0
        );
    }

    #[test]
    fn f64_divide_opcode() {
        math_f64_op_test!(Opcode::DivideF64.byte(),
            9.0, 4.0, 2.25
        );
    }

    #[test]
    fn f64_equals_opcode() {
        condition_f64_op_test!(Opcode::EqualF64.byte(),
            10.0, 10.0,
            10.0, 20.0
        );
    }

    #[test]
    fn f64_not_equals_opcode() {
        condition_f64_op_test!(Opcode::NotEqualF64.byte(),
            10.0, 20.0,
            10.0, 10.0
        );
    }

    #[test]
    fn f64_greater_than_opcode() {
        condition_f64_op_test!(Opcode::GreaterThanF64.byte(),
            10.0, 5.0,
            10.0, 10.0
        );
    }

    #[test]
    fn f64_less_than_opcode() {
        condition_f64_op_test!(Opcode::LessThanF64.byte(),
            5.0, 10.0,
            10.0, 5.0
        );
    }

    #[test]
    fn f64_greater_than_or_equal_opcode() {
        condition_f64_op_test!(Opcode::GreaterThanOrEqualF64.byte(),
            10.0, 10.0,
            5.0, 10.0
        );
    }

    #[test]
    fn f64_less_than_or_equal_opcode() {
        condition_f64_op_test!(Opcode::LessThanOrEqualF64.byte(),
            10.0, 10.0,
            10.0, 5.0
        );
    }


    #[test]
    fn jump_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 4;
        test_vm.program = vec![Opcode::Jump.byte(), 0];
        test_vm.run_once();

        // We should have jumped to byte 4 (because register 0 is 4)
        assert_eq!(test_vm.pc, 4);
    }
    
    #[test]
    fn jump_forward_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 2;
        test_vm.program = vec![Opcode::JumpForward.byte(), 0];
        test_vm.run_once();
        
        // We should have jumped forward 2 bytes (because register 0 is 2)
        assert_eq!(test_vm.pc, 4);
    }
    
    #[test]
    fn jump_backward_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.pc = 4;
        test_vm.registers[0] = 6;
        test_vm.program = vec![0, 0, 0, 0, Opcode::JumpBackward.byte(), 0];
        test_vm.run_once();

        // We should have jumped backward 6 bytes (because register 0 is 6)
        // The amount to jump is 6 because we also have to include the jump back instruction
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn jump_if_equal_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[1] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![Opcode::JumpIfEqual.byte(), 1];
        test_vm.run_once();
        
        // We should appear on byte seven, as that's where register 1 sends us
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn move_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.registers[1] = 1;
        test_vm.program = vec![Opcode::Move.byte(), 1, 0];
        test_vm.run_once();
        
        // We should appear on byte seven, as that's where register 1 sends us
        assert_eq!(test_vm.registers[0], 1);
    }
}