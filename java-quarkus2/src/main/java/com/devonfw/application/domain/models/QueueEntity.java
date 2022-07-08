package com.devonfw.application.domain.models;

import java.util.Date;

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

  @Temporal(TemporalType.TIMESTAMP)
  private Date attentionTime;

  @Temporal(TemporalType.TIMESTAMP)
  private Date minAttentionTime;

  private Boolean active;
}
