package club.marzipan.javacarrentals;

import java.time.ZonedDateTime;
import java.util.UUID;

import javax.persistence.Column;
import javax.persistence.Entity;
import javax.persistence.EnumType;
import javax.persistence.Enumerated;
import javax.persistence.Id;
import javax.persistence.Table;

@Entity
@Table(name = "account")
public class AccountEntity {

    public enum Status {
        unverified, active, disabled, deleted
    }

    @Id
    public UUID id;

    @Column(name = "created")
    public ZonedDateTime created;

    @Column(name = "updated")
    public ZonedDateTime updated;

    @Column(name = "username")
    public String username;

    @Column(name = "email")
    public String email;

    @Column(name = "password_hash")
    public String passwordHash;

    @Column(name = "status")
    @Enumerated(EnumType.STRING)
    public Status status;

}
