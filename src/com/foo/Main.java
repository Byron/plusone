package com.foo;

import java.util.Optional;

public class Main {

    public static void main(String[] args) {
        int exitStatus = 0;
        for (String arg : args) {
            Optional<Integer> incrementedInput = Utils.parseIntSafely(arg).map(value -> value + 1);

            if (incrementedInput.isPresent()) {
                System.out.println(incrementedInput.get());
            } else {
                System.err.println(arg + " is not a number");
                exitStatus = 2;
            }
        }
        System.exit(exitStatus);
    }

}
