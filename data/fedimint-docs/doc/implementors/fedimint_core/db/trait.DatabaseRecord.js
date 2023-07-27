(function() {var implementors = {
"fedimint_client":[["impl&lt;S&gt; DatabaseRecord for <a class=\"struct\" href=\"fedimint_client/db/struct.ClientSecretKey.html\" title=\"struct fedimint_client::db::ClientSecretKey\">ClientSecretKey</a>&lt;S&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"fedimint_client/secret/trait.RootSecretStrategy.html\" title=\"trait fedimint_client::secret::RootSecretStrategy\">RootSecretStrategy</a>,</span>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client/db/struct.OperationLogKey.html\" title=\"struct fedimint_client::db::OperationLogKey\">OperationLogKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client/db/struct.ChronologicalOperationLogKey.html\" title=\"struct fedimint_client::db::ChronologicalOperationLogKey\">ChronologicalOperationLogKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client/db/struct.CachedApiVersionSetKey.html\" title=\"struct fedimint_client::db::CachedApiVersionSetKey\">CachedApiVersionSetKey</a>"]],
"fedimint_client_legacy":[["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/db/struct.ClientSecretKey.html\" title=\"struct fedimint_client_legacy::db::ClientSecretKey\">ClientSecretKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.OutgoingPaymentKey.html\" title=\"struct fedimint_client_legacy::ln::db::OutgoingPaymentKey\">OutgoingPaymentKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.OutgoingPaymentClaimKey.html\" title=\"struct fedimint_client_legacy::ln::db::OutgoingPaymentClaimKey\">OutgoingPaymentClaimKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.OutgoingContractAccountKey.html\" title=\"struct fedimint_client_legacy::ln::db::OutgoingContractAccountKey\">OutgoingContractAccountKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.ConfirmedInvoiceKey.html\" title=\"struct fedimint_client_legacy::ln::db::ConfirmedInvoiceKey\">ConfirmedInvoiceKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.LightningGatewayKey.html\" title=\"struct fedimint_client_legacy::ln::db::LightningGatewayKey\">LightningGatewayKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.NoteKey.html\" title=\"struct fedimint_client_legacy::mint::db::NoteKey\">NoteKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.PendingNotesKey.html\" title=\"struct fedimint_client_legacy::mint::db::PendingNotesKey\">PendingNotesKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.OutputFinalizationKey.html\" title=\"struct fedimint_client_legacy::mint::db::OutputFinalizationKey\">OutputFinalizationKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.NextECashNoteIndexKey.html\" title=\"struct fedimint_client_legacy::mint::db::NextECashNoteIndexKey\">NextECashNoteIndexKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.NotesPerDenominationKey.html\" title=\"struct fedimint_client_legacy::mint::db::NotesPerDenominationKey\">NotesPerDenominationKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_client_legacy/wallet/db/struct.PegInKey.html\" title=\"struct fedimint_client_legacy::wallet::db::PegInKey\">PegInKey</a>"]],
"fedimint_core":[],
"fedimint_ln_common":[["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.ContractKey.html\" title=\"struct fedimint_ln_common::db::ContractKey\">ContractKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.ContractUpdateKey.html\" title=\"struct fedimint_ln_common::db::ContractUpdateKey\">ContractUpdateKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.OfferKey.html\" title=\"struct fedimint_ln_common::db::OfferKey\">OfferKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.ProposeDecryptionShareKey.html\" title=\"struct fedimint_ln_common::db::ProposeDecryptionShareKey\">ProposeDecryptionShareKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.AgreedDecryptionShareKey.html\" title=\"struct fedimint_ln_common::db::AgreedDecryptionShareKey\">AgreedDecryptionShareKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.LightningGatewayKey.html\" title=\"struct fedimint_ln_common::db::LightningGatewayKey\">LightningGatewayKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.BlockHeightVoteKey.html\" title=\"struct fedimint_ln_common::db::BlockHeightVoteKey\">BlockHeightVoteKey</a>"]],
"fedimint_mint_common":[["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.NonceKey.html\" title=\"struct fedimint_mint_common::db::NonceKey\">NonceKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.ProposedPartialSignatureKey.html\" title=\"struct fedimint_mint_common::db::ProposedPartialSignatureKey\">ProposedPartialSignatureKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.ReceivedPartialSignatureKey.html\" title=\"struct fedimint_mint_common::db::ReceivedPartialSignatureKey\">ReceivedPartialSignatureKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.OutputOutcomeKey.html\" title=\"struct fedimint_mint_common::db::OutputOutcomeKey\">OutputOutcomeKey</a>"],["impl DatabaseRecord for <a class=\"enum\" href=\"fedimint_mint_common/db/enum.MintAuditItemKey.html\" title=\"enum fedimint_mint_common::db::MintAuditItemKey\">MintAuditItemKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.EcashBackupKey.html\" title=\"struct fedimint_mint_common::db::EcashBackupKey\">EcashBackupKey</a>"]],
"fedimint_server":[["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_server/db/struct.AcceptedTransactionKey.html\" title=\"struct fedimint_server::db::AcceptedTransactionKey\">AcceptedTransactionKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_server/db/struct.EpochHistoryKey.html\" title=\"struct fedimint_server::db::EpochHistoryKey\">EpochHistoryKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_server/db/struct.LastEpochKey.html\" title=\"struct fedimint_server::db::LastEpochKey\">LastEpochKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_server/db/struct.ClientConfigSignatureKey.html\" title=\"struct fedimint_server::db::ClientConfigSignatureKey\">ClientConfigSignatureKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_server/db/struct.ClientConfigSignatureShareKey.html\" title=\"struct fedimint_server::db::ClientConfigSignatureShareKey\">ClientConfigSignatureShareKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_server/db/struct.ConsensusUpgradeKey.html\" title=\"struct fedimint_server::db::ConsensusUpgradeKey\">ConsensusUpgradeKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_server/db/struct.ClientConfigDownloadKey.html\" title=\"struct fedimint_server::db::ClientConfigDownloadKey\">ClientConfigDownloadKey</a>"]],
"fedimint_wallet_common":[["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.BlockHashKey.html\" title=\"struct fedimint_wallet_common::db::BlockHashKey\">BlockHashKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.UTXOKey.html\" title=\"struct fedimint_wallet_common::db::UTXOKey\">UTXOKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.UnsignedTransactionKey.html\" title=\"struct fedimint_wallet_common::db::UnsignedTransactionKey\">UnsignedTransactionKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PendingTransactionKey.html\" title=\"struct fedimint_wallet_common::db::PendingTransactionKey\">PendingTransactionKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PegOutTxSignatureCI.html\" title=\"struct fedimint_wallet_common::db::PegOutTxSignatureCI\">PegOutTxSignatureCI</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PegOutBitcoinTransaction.html\" title=\"struct fedimint_wallet_common::db::PegOutBitcoinTransaction\">PegOutBitcoinTransaction</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.BlockHeightVoteKey.html\" title=\"struct fedimint_wallet_common::db::BlockHeightVoteKey\">BlockHeightVoteKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.FeeRateVoteKey.html\" title=\"struct fedimint_wallet_common::db::FeeRateVoteKey\">FeeRateVoteKey</a>"],["impl DatabaseRecord for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PegOutNonceKey.html\" title=\"struct fedimint_wallet_common::db::PegOutNonceKey\">PegOutNonceKey</a>"]],
"ln_gateway":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"ln_gateway/db/struct.FederationIdKey.html\" title=\"struct ln_gateway::db::FederationIdKey\">FederationIdKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"ln_gateway/db/struct.FederationRegistrationKey.html\" title=\"struct ln_gateway::db::FederationRegistrationKey\">FederationRegistrationKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"ln_gateway/db/struct.GatewayPublicKey.html\" title=\"struct ln_gateway::db::GatewayPublicKey\">GatewayPublicKey</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()