package club.marzipan;

import java.sql.SQLException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.hibernate.JDBCException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestController;

@RestController
@ControllerAdvice
public class ErrorHandler {

    private static final Logger logger = LoggerFactory.getLogger(ErrorHandler.class);

    private static final MediaType plaintext = MediaType.valueOf("text/plain; charset=utf-8");

    @ExceptionHandler(MethodArgumentNotValidException.class)
    public ResponseEntity<Map<String, List<String>>> handleValidationExceptions(
            MethodArgumentNotValidException methodArgumentNotValidException) {

        logger.warn(methodArgumentNotValidException.getMessage());
        Map<String, List<String>> errors = new HashMap<>();

        methodArgumentNotValidException.getBindingResult().getFieldErrors().forEach((field_error) -> {
            List<String> messages = errors.computeIfAbsent(field_error.getField(), field -> new ArrayList<>());
            messages.add(field_error.getDefaultMessage());
        });

        return ResponseEntity.badRequest().body(errors);
    }

    @ExceptionHandler(JDBCException.class)
    public ResponseEntity<String> handleSQLException(JDBCException jdbcException) {

        SQLException sqlException = jdbcException.getSQLException();
        HttpStatus status;

        switch (sqlException.getSQLState()) {
            // https://www.postgresql.org/docs/current/errcodes-appendix.html

            case "23505": // unique violation
                status = HttpStatus.CONFLICT;
                logger.warn(sqlException.getMessage());
                break;

            case "23000": // integrity constraint violation
            case "23503": // foreign key violation
                status = HttpStatus.BAD_REQUEST;
                logger.warn(sqlException.getMessage());
                break;

            default:
                status = HttpStatus.INTERNAL_SERVER_ERROR;
                logger.error(sqlException.getMessage());
        }

        return ResponseEntity.status(status).contentType(plaintext).body(sqlException.getMessage());
    }

}
