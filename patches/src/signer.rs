// reference
// https://github.com/solana-foundation/anchor/pull/3340
// https://github.com/metaplex-foundation/mpl-candy-machine/pull/76/files
//! Explicit wrapper for AccountInfo types to emphasize
//! that no checks are performed
use anchor_lang::error::ErrorCode;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::instruction::AccountMeta;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{Accounts, AccountsExit, Key, Result, ToAccountInfos, ToAccountMetas};
use std::collections::BTreeSet;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Signer<'info> {
    info: &'info AccountInfo<'info>,
}

impl<'info> Signer<'info> {
    fn new(info: &'info AccountInfo<'info>) -> Signer<'info> {
        Self { info }
    }

    /// Deserializes the given `info` into a `Signer`.
    #[inline(never)]
    pub fn try_from(info: &'info AccountInfo<'info>) -> Result<Signer<'info>> {
        if !info.is_signer {
            return Err(ErrorCode::AccountNotSigner.into());
        }
        Ok(Signer::new(info))
    }
}

impl<'info, B> Accounts<'info, B> for Signer<'info> {
    #[inline(never)]
    fn try_accounts(
        _program_id: &Pubkey,
        accounts: &mut &'info [AccountInfo<'info>],
        _ix_data: &[u8],
        _bumps: &mut B,
        _reallocs: &mut BTreeSet<Pubkey>,
    ) -> Result<Self> {
        if accounts.is_empty() {
            return Err(ErrorCode::AccountNotEnoughKeys.into());
        }
        let account = &accounts[0];
        *accounts = &accounts[1..];
        Signer::try_from(account)
    }
}

impl<'info> AccountsExit<'info> for Signer<'info> {}

impl ToAccountMetas for Signer<'_> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.info.is_signer);
        let meta = match self.info.is_writable {
            false => AccountMeta::new_readonly(*self.info.key, is_signer),
            true => AccountMeta::new(*self.info.key, is_signer),
        };
        vec![meta]
    }
}

impl<'info> ToAccountInfos<'info> for Signer<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.info.clone()]
    }
}

impl<'info> AsRef<AccountInfo<'info>> for Signer<'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

impl<'info> Deref for Signer<'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}

impl Key for Signer<'_> {
    fn key(&self) -> Pubkey {
        *self.info.key
    }
}
