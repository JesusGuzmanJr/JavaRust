package club.marzipan.javacarrentals;

import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.http.ResponseEntity;
import org.springframework.http.HttpStatus;

import java.util.UUID;
import java.util.Optional;

import org.springframework.beans.factory.annotation.Autowired;

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
            return new ResponseEntity<>(account, HttpStatus.OK);
        } else {
            return new ResponseEntity<>(null, HttpStatus.NOT_FOUND);
        }
    }

}
