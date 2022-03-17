package com.devonfw.application.jtqj.general.common.base.test;

import com.devonfw.module.test.common.base.SubsystemDbTest;

import com.devonfw.application.jtqj.SpringBootApp;

import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.test.context.SpringBootTest.WebEnvironment;

/**
 * Abstract base class for {@link SubsystemTest}s of this application.
 */
@SpringBootTest(classes = { SpringBootApp.class }, webEnvironment = WebEnvironment.RANDOM_PORT)
public abstract class ApplicationSubsystemTest extends SubsystemDbTest {

}
