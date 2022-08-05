package com.devonfw.application.domain.models;

import java.sql.Timestamp;

import javax.persistence.CascadeType;
import javax.persistence.Column;
import javax.persistence.Entity;
import javax.persistence.FetchType;
import javax.persistence.JoinColumn;
import javax.persistence.ManyToOne;
import javax.persistence.OneToOne;
import javax.persistence.Table;

import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

@Entity
@Table(name = "AccessCode")
@Getter
@Setter
@ToString
public class AccessCodeEntity extends ApplicationPersistenceEntity {

  private Timestamp creationTime;

  private Timestamp startTime;

  private Timestamp endTime;

  @OneToOne(cascade = {CascadeType.DETACH}, fetch = FetchType.EAGER)
  @JoinColumn(name = "idVisitor", insertable = false, updatable = false)
  private VisitorEntity visitor;

  @ManyToOne(cascade = {CascadeType.DETACH}, fetch = FetchType.EAGER)
  @JoinColumn(name = "idQueue", insertable = false, updatable = false)
  private QueueEntity queue;

  @Column(name = "idVisitor")
  private Long visitorId;

  @Column(name = "idQueue")
  private Long queueId;
}
