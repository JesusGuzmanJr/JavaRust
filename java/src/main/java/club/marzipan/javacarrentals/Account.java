package club.marzipan.javacarrentals;

import java.time.ZonedDateTime;
import java.util.UUID;

public class Account {

    public enum Status {
        unverified, active, disabled, deleted
    }

    public UUID id;
    public ZonedDateTime created;
    public ZonedDateTime updated;
    public String username;
    public String email;
    public String passwordHash;
    public Status status;
}
