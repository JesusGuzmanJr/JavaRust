package club.marzipan.javacarrentals;

import java.security.NoSuchAlgorithmException;
import java.security.SecureRandom;
import java.util.Arrays;

import javax.annotation.PostConstruct;

import org.bouncycastle.crypto.generators.Argon2BytesGenerator;
import org.bouncycastle.crypto.params.Argon2Parameters;
import org.springframework.stereotype.Component;

@Component
public class PasswordHasher {

    // Argon2 with default params
    private static final int algorithm = Argon2Parameters.ARGON2_id;
    private static final int version = 13;
    private static final int timeCost = 5;
    private static final int memoryCostPowOfTwo = 12; // 2^12= 4096
    private static final int parallelism = 2;
    private static final int saltLength = 16;
    private static final int hashLength = 32;

    private static SecureRandom generator;

    @PostConstruct
    private void init() throws NoSuchAlgorithmException {
        generator = SecureRandom.getInstance("SHA1PRNG");
    }

    public byte[] createSalt() {
        return generator.generateSeed(saltLength);
    }

    public byte[] hash(String password, byte[] salt) {
        Argon2Parameters.Builder builder = new Argon2Parameters.Builder(algorithm).withVersion(version)
                .withIterations(timeCost).withMemoryPowOfTwo(memoryCostPowOfTwo).withParallelism(parallelism)
                .withSalt(salt);

        Argon2BytesGenerator argon2 = new Argon2BytesGenerator();
        argon2.init(builder.build());

        byte[] hash = new byte[hashLength];

        argon2.generateBytes(password.getBytes(), hash, 0, hash.length);

        return hash;
    }

    public boolean verify(String password, byte[] password_hash, byte[] password_salt) {
        byte[] testHash = hash(password, password_salt);
        return Arrays.equals(testHash, password_hash);
    }

}
