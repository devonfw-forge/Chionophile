package com.devonfw.application.api.mapper;

import com.devonfw.module.json.common.base.ObjectMapperFactory;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.quarkus.jackson.ObjectMapperCustomizer;

import javax.enterprise.inject.Instance;
import javax.inject.Singleton;

public class RegisterCustomModuleCustomizer {
    // Replaces the CDI producer for ObjectMapper built into Quarkus
    @Singleton
    ObjectMapper objectMapper(Instance<ObjectMapperCustomizer> customizers) {
        ObjectMapper mapper = new ObjectMapperFactory().createInstance();
        System.out.println("My CUSTOMIZER CALLED");
        for (ObjectMapperCustomizer customizer : customizers) {
            customizer.customize(mapper);
        }
        return mapper;
    }
}