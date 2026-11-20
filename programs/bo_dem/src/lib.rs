use anchor_lang::prelude::*;

declare_id!("Fubg1GAo83Yfq5HEwd4pmfs2RqbmdbHA8yUuiTu1ApZh"); // Sẽ cập nhật ID thật ở bước 3

#[program]
pub mod bo_dem {
    use super::*;

    // Hàm 1: Tạo một biến đếm mới trên blockchain với giá trị ban đầu = 0
    pub fn khoi_tao(ctx: Context<KhoiTao>) -> Result<()> {
        let tai_khoan_dem = &mut ctx.accounts.tai_khoan_dem;
        tai_khoan_dem.gia_tri = 0;
        msg!("Đã khởi tạo bộ đếm trên Solana! Giá trị = 0");
        Ok(())
    }

    // Hàm 2: Tăng giá trị biến đếm trên blockchain lên 1 đơn vị
    pub fn tang_len(ctx: Context<TangLen>) -> Result<()> {
        let tai_khoan_dem = &mut ctx.accounts.tai_khoan_dem;
        tai_khoan_dem.gia_tri += 1;
        msg!("Đã tăng bộ đếm! Giá trị hiện tại = {}", tai_khoan_dem.gia_tri);
        Ok(())
    }
}

// Khai báo cấu trúc lưu trữ dữ liệu trên Solana (giống như tạo bảng trong Database)
#[account]
pub struct DuLieuDem {
    pub gia_tri: u64, // Số nguyên không dấu 64-bit
}

// Khai báo quyền hạn cho hàm khoi_tao
#[derive(Accounts)]
pub struct KhoiTao<'info> {
    #[account(init, payer = nguoi_tra_phi, space = 8 + 8)]
    pub tai_khoan_dem: Account<'info, DuLieuDem>,
    #[account(mut)]
    pub nguoi_tra_phi: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Khai báo quyền hạn cho hàm tang_len
#[derive(Accounts)]
pub struct TangLen<'info> {
    #[account(mut)]
    pub tai_khoan_dem: Account<'info, DuLieuDem>,
}