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
}