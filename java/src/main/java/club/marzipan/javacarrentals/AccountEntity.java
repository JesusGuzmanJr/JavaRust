package club.marzipan.javacarrentals;

import java.sql.Date;
import java.util.UUID;

import javax.persistence.Column;
import javax.persistence.Entity;
import javax.persistence.EnumType;
import javax.persistence.Enumerated;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;
import javax.persistence.Table;

import club.marzipan.javacarrentals.Account.Status;

@Entity
@Table(name = "account")
public class AccountEntity {

    @Id
    @GeneratedValue(generator = "uuid4")
    UUID id;

    @Column(name = "created")
    Date created;

    @Column(name = "updated")
    Date updated;

    @Column(name = "username")
    String username;

    @Column(name = "email")
    String email;

    @Column(name = "password_hash")
    String passwordHash;

    @Column(name = "status")
    @Enumerated(EnumType.STRING)
    Status status;

}
