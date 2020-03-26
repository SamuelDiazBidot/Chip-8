use crate::cpu::CPU;

#[cfg(test)]
mod tests{
    use super::*;

    fn testing_cpu(first_byte: u8, second_byte: u8) -> CPU{
        let mut cpu = CPU::new();
        cpu.memory[0x200] = first_byte;
        cpu.memory[0x201] = second_byte;
        cpu
    }

    #[test]
    fn test_00e0() {
        let mut cpu = testing_cpu(0x00, 0xE0);
        cpu.graphics = [[1; 64]; 32];
        cpu.emulate_cycle();
        for y in 0..32 {
            for x in 0..64 {
                assert_eq!(cpu.graphics[y][x], 0);
            }
        }
    }

    #[test]
    fn test_00ee() {
        let mut cpu = testing_cpu(0x00, 0xEE);
        cpu.sp = 2;
        cpu.stack[1] = 0xFFFF;
        cpu.emulate_cycle();
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xFFFF);
    }

    #[test]
    fn test_1nnn() {
        let mut cpu = testing_cpu(0x1F,0xFF);
        cpu.emulate_cycle();
        assert_eq!(cpu.pc, 0x0FFF);
    }

    #[test]
    fn test_2nnn() {
        let mut cpu = testing_cpu(0x2F,0xFF);
        cpu.emulate_cycle();
        assert_eq!(cpu.stack[0], 0x202);
        assert_eq!(cpu.sp, 1); 
        assert_eq!(cpu.pc, 0x0FFF);
    }

    #[test]
    fn test_3xkk() {
        let mut cpu = testing_cpu(0x3F,0x02);
        cpu.v[0xF] = 0x02;
        cpu.emulate_cycle();
        assert_eq!(cpu.pc, 0x204);
        let mut cpu1 = testing_cpu(0x3F,0x02);
        cpu1.v[0xF] = 0x01;
        cpu1.emulate_cycle();
        assert_eq!(cpu1.pc, 0x202);
    }

    #[test]
    fn test_4xkk() {
        let mut cpu = testing_cpu(0x4F,0x02);
        cpu.v[0xF] = 0x01;
        cpu.emulate_cycle();
        assert_eq!(cpu.pc, 0x204);
        let mut cpu1 = testing_cpu(0x4F,0x02);
        cpu1.v[0xF] = 0x02;
        cpu1.emulate_cycle();
        assert_eq!(cpu1.pc, 0x202);
    }

    #[test]
    fn test_5xy0() {
        let mut cpu = testing_cpu(0x52,0x30);
        cpu.v[2] = 1;
        cpu.v[3] = 1;
        cpu.emulate_cycle();
        assert_eq!(cpu.pc, 0x204);
        let mut cpu1 = testing_cpu(0x52,0x30);
        cpu1.v[2] = 2;
        cpu1.v[3] = 1;
        cpu1.emulate_cycle();
        assert_eq!(cpu1.pc, 0x202);
    }

    #[test]
    fn test_6xkk() {
        let mut cpu = testing_cpu(0x60,0xFF);
        cpu.emulate_cycle();
        assert_eq!(cpu.v[0], 0xFF);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_7xkk() {
        let mut cpu = testing_cpu(0x70,0x02);
        cpu.emulate_cycle();
        assert_eq!(cpu.v[0], 0x02);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_8xy0() {
        let mut cpu = testing_cpu(0x80,0x10);
        cpu.v[0] = 0;
        cpu.v[1] = 1;
        cpu.emulate_cycle();
        assert_eq!(cpu.v[0], cpu.v[1]);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_8xy1() {
        let mut cpu = testing_cpu(0x80,0x11);
        cpu.v[0] = 0;
        cpu.v[1] = 1;
        cpu.emulate_cycle();
        assert_eq!(cpu.v[0], 0 | 1);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_8xy2() {
        let mut cpu = testing_cpu(0x80,0x12);
        cpu.v[0] = 0;
        cpu.v[1] = 1;
        cpu.emulate_cycle();
        assert_eq!(cpu.v[0], 0 & 1);
        assert_eq!(cpu.pc, 0x202); 
    }

    #[test]
    fn test_8xy3() {
        let mut cpu = testing_cpu(0x80,0x13);
        cpu.v[0] = 0;
        cpu.v[1] = 1;
        cpu.emulate_cycle();
        assert_eq!(cpu.v[0], 0 ^ 1);
        assert_eq!(cpu.pc, 0x202); 
    }

    #[test]
    fn test_8xy4() {
        let mut cpu = testing_cpu(0x80,0x14);
        cpu.v[0] = 0;
        cpu.v[1] = 1;
        cpu.emulate_cycle();
        assert_eq!(cpu.v[0], 0 + 1);
        assert_eq!(cpu.v[0xF], 0);
        assert_eq!(cpu.pc, 0x202);
        let mut cpu1 = testing_cpu(0x80,0x14);
        cpu1.v[0] = 0xFF;
        cpu1.v[1] = 1;
        cpu1.emulate_cycle();
        assert_eq!(cpu1.v[0], 0);
        assert_eq!(cpu1.v[0xF], 1);
        assert_eq!(cpu1.pc, 0x202);
    }
}