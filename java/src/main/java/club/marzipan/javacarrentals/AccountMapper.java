package club.marzipan.javacarrentals;

import org.springframework.stereotype.Component;

@Component
public class AccountMapper {
    public Account mapEntityToApi(AccountEntity accountEntity) {
        if (accountEntity == null) {
            return null;
        }
        Account account = new Account();
        account.setId(accountEntity.getId());
        account.setCreated(accountEntity.getCreated());
        account.setUpdated(accountEntity.getUpdated());
        account.setUsername(accountEntity.getUsername());
        account.setEmail(accountEntity.getEmail());
        account.setPasswordHash(accountEntity.getPasswordHash());
        account.setPasswordSalt(accountEntity.getPasswordSalt());
        if (accountEntity.getStatus() != null) {
            switch (accountEntity.getStatus()) {
                case active:
                    account.setStatus(Account.Status.active);
                    break;
                case unverified:
                    account.setStatus(Account.Status.unverified);
                    break;
                case disabled:
                    account.setStatus(Account.Status.disabled);
                    break;
                case deleted:
                    account.setStatus(Account.Status.deleted);
                    break;
            }
        }
        return account;
    }
}
