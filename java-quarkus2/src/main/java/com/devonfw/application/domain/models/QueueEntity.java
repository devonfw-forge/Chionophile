package com.devonfw.application.domain.models;

import java.sql.Timestamp;

import javax.persistence.Entity;
import javax.persistence.Table;
import javax.persistence.Temporal;
import javax.persistence.TemporalType;

import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

@Entity
@Table(name = "DailyQueue")
@Getter
@Setter
@ToString
public class QueueEntity extends ApplicationPersistenceEntity {
  private String name;

  private String logo;

  private String currentNumber;

  private Timestamp attentionTime;

  private Timestamp minAttentionTime;

  private Boolean active;
}
