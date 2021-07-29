package club.marzipan.javacarrentals;

import java.time.ZoneOffset;
import java.time.ZonedDateTime;
import java.util.Optional;
import java.util.UUID;

import javax.validation.Valid;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class AccountEndpoint {

    @Autowired
    private AccountRepository accountRepository;

    @Autowired
    private AccountMapper accountMapper;

    @GetMapping("/account/{id}")
    public ResponseEntity<Account> getAccount(@PathVariable UUID id) {
        Optional<AccountEntity> accountEntity = accountRepository.findById(id);
        if (accountEntity.isPresent()) {
            Account account = accountMapper.mapEntityToApi(accountEntity.get());
            return ResponseEntity.ok().body(account);
        } else {
            return ResponseEntity.notFound().build();
        }
    }

    @PostMapping("/account")
    public ResponseEntity<Account> createAccount(@Valid @RequestBody CreateAccount createAccount) {
        ZonedDateTime now = ZonedDateTime.now().withZoneSameInstant(ZoneOffset.UTC);

        AccountEntity accountEntity = new AccountEntity();
        accountEntity.id = UUID.randomUUID();
        accountEntity.created = now;
        accountEntity.updated = now;
        accountEntity.username = createAccount.username;
        accountEntity.email = createAccount.email;
        accountEntity.passwordHash = "";
        accountEntity.status = AccountEntity.Status.unverified;

        accountRepository.saveAndFlush(accountEntity);
        return ResponseEntity.ok().body(accountMapper.mapEntityToApi(accountEntity));
    }

}
