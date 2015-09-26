package com.foo;

import java.util.Optional;

public class Utils {
    public static Optional<Integer> incrementString(String arg) {
        try {
            return Optional.of(Integer.valueOf(arg) + 1);
        } catch (NumberFormatException e) {
            return Optional.empty();
        }
    }
}
