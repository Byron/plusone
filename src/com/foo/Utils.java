package com.foo;

import java.util.Optional;

public class Utils {
    static Optional<Integer> parseIntSafely(String arg) {
        try {
            return Optional.of(Integer.valueOf(arg));
        } catch (NumberFormatException e) {
            return Optional.empty();
        }
    }
}
