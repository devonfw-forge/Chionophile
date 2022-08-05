package com.devonfw.application.api.mapper;

import java.text.SimpleDateFormat;

import javax.inject.Named;

import com.fasterxml.jackson.annotation.JsonInclude.Include;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;

import io.quarkus.jackson.ObjectMapperCustomizer;

/**
 * The MappingFactory class to resolve polymorphic conflicts within the .0.1
 * application.
 */
@Named("ApplicationObjectMapperFactory")
public class ApplicationObjectMapperFactory implements ObjectMapperCustomizer {
  @Override
  public void customize(ObjectMapper objectMapper) {
    // omit properties in JSON that are null
    objectMapper.setSerializationInclusion(Include.NON_NULL);
    // Write legacy date/calendar as readable text instead of numeric value
    // See
    // https://fasterxml.github.io/jackson-databind/javadoc/2.6/com/fasterxml/jackson/databind/SerializationFeature.html#WRITE_DATES_AS_TIMESTAMPS
    objectMapper.configure(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS, false);
    objectMapper.setDateFormat(new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss.S"));
    // ignore unknown properties in JSON to prevent errors
    // e.g. when the service has been updated/extended but the calling REST client
    // is not yet updated
    // see
    // https://github.com/devonfw/devon4j/blob/develop/documentation/guide-service-layer.asciidoc#versioning
    objectMapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
  }
}
