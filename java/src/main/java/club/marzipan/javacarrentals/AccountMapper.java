package club.marzipan.javacarrentals;

import org.springframework.stereotype.Component;

@Component
public class AccountMapper {
    public Account mapEntityToApi(AccountEntity accountEntity) {
        if (accountEntity == null) {
            return null;
        }
        Account account = new Account();
        account.id = accountEntity.id;
        account.created = accountEntity.created;
        account.updated = accountEntity.updated;
        account.username = accountEntity.username;
        account.email = accountEntity.email;
        account.passwordHash = accountEntity.passwordHash;
        if (accountEntity.status != null) {
            switch (accountEntity.status) {
                case active:
                    account.status = Account.Status.active;
                    break;
                case unverified:
                    account.status = Account.Status.unverified;
                    break;
                case disabled:
                    account.status = Account.Status.disabled;
                    break;
                case deleted:
                    account.status = Account.Status.deleted;
                    break;
            }
        }
        return account;
    }
}
