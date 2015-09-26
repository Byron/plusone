package com.foo.tests;

import org.junit.Test;
import com.foo.Utils;

import static org.hamcrest.Matchers.equalTo;
import static org.hamcrest.Matchers.is;
import static org.junit.Assert.*;

public class UtilsTest {

    @Test
    public void testParseIntSafely() throws Exception {
        assertThat(Utils.incrementString("5").get(), equalTo(6));
        assertThat(Utils.incrementString("-1").get(), equalTo(0));
        assertThat(Utils.incrementString("foo").isPresent(), is(false));
    }
}