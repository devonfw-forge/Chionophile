package com.devonfw.application.domain.models;

import javax.persistence.GeneratedValue;
import javax.persistence.GenerationType;
import javax.persistence.Id;
import javax.persistence.MappedSuperclass;
import javax.persistence.Transient;
import javax.persistence.Version;

import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

/**
 * Abstract base class for all {@link PersistenceEntity persistence entities}
 * with an {@link #getId() id} and a
 * {@link #getModificationCounter() modificationCounter} (version) field. All
 * persistence entities of this application
 * should inherit from this class. It is using JPA annotations at the getters
 * what has several advantages but also
 * implies that you have to annotate transient getter methods with the
 * {@link Transient} annotation.
 */
@MappedSuperclass
@Getter
@Setter
@ToString(includeFieldNames = true, of = { "id" })
@EqualsAndHashCode(of = { "id" })
public abstract class ApplicationPersistenceEntity {

  @Id
  @GeneratedValue(strategy = GenerationType.IDENTITY)
  Long id;

  @Version
  private Integer modificationCounter;
}
