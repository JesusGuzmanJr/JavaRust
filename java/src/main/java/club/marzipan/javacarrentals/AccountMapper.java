package club.marzipan.javacarrentals;

import org.springframework.stereotype.Component;

@Component
public class AccountMapper {
    public Account mapEntityToApi(AccountEntity accountEntity) {
        if (accountEntity == null) {
            return null;
        }
        Account account = new Account();
        account.setId(accountEntity.id);
        account.setCreated(accountEntity.created);
        account.setUpdated(accountEntity.updated);
        account.setUsername(accountEntity.username);
        account.setEmail(accountEntity.email);
        account.setPasswordHash(accountEntity.passwordHash);
        account.setStatus(accountEntity.status);
        return account;
    }
}
