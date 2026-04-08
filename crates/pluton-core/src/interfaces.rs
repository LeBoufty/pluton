//! mail interfaces

use std::collections::HashMap;

use pluton_mail::{
  interface::{AnyMailInterface, MailInterface},
  protocol::{imap::ImapIncoming, smtp::SmtpOutgoing},
};

use crate::{
  PlutonCoreError, PlutonCoreResult,
  accounts::{AccountID, IncomingConnection, OutgoingConnection, manager::AccountManager},
};

/// general manager
pub struct MailInterfaceManager(HashMap<AccountID, AnyMailInterface>);

enum IncomingProtocolImpls {
  IMAP(ImapIncoming),
}

enum OutgoingProtocolImpls {
  SMTP(SmtpOutgoing),
}

macro_rules! outgoing_in_match {
  ($inc:ident, $out:ident) => {
    match $out {
      OutgoingProtocolImpls::SMTP(smtp) => MailInterface::new($inc, smtp),
    }
  };
}

impl MailInterfaceManager {
  /// main instanciator used in pluton
  pub fn from_accounts(accounts: &AccountManager) -> PlutonCoreResult<Self> {
    let mut hashmap = HashMap::new();

    for (acc_id, acc) in &accounts.get_accounts().0 {
      let incoming = match &acc.incoming {
        IncomingConnection::IMAP(imap) => {
          let password = accounts.get_password(&format!("imap-{}", acc_id))?;
          let imp = ImapIncoming::new_tls(&imap.server, imap.port, &imap.address, &password)?;
          IncomingProtocolImpls::IMAP(imp)
        }
      };

      let outgoing = match &acc.outgoing {
        OutgoingConnection::SMTP(smtp) => {
          let password = accounts.get_password(&format!("smtp-{}", acc_id))?;
          let imp = SmtpOutgoing::new(&smtp.server, &smtp.address, &password)?;
          OutgoingProtocolImpls::SMTP(imp)
        }
      };

      let interface: AnyMailInterface = Box::new(match incoming {
        IncomingProtocolImpls::IMAP(imap) => outgoing_in_match!(imap, outgoing),
      }?);
      hashmap.insert(*acc_id, interface);
    }

    Ok(Self(hashmap))
  }

  pub fn get(&self, account_id: &AccountID) -> PlutonCoreResult<&AnyMailInterface> {
    let v = self
      .0
      .get(account_id)
      .ok_or(PlutonCoreError::AccountNotFound)?;
    Ok(v)
  }

  pub fn get_mut(&mut self, account_id: &AccountID) -> PlutonCoreResult<&mut AnyMailInterface> {
    let v = self
      .0
      .get_mut(account_id)
      .ok_or(PlutonCoreError::AccountNotFound)?;
    Ok(v)
  }
}
