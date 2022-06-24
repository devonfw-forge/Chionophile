package com.devonfw.application.domain.models;

import java.sql.Timestamp;

import javax.persistence.CascadeType;
import javax.persistence.Entity;
import javax.persistence.FetchType;
import javax.persistence.JoinColumn;
import javax.persistence.ManyToOne;
import javax.persistence.OneToOne;
import javax.persistence.Table;
import javax.persistence.Temporal;
import javax.persistence.TemporalType;
import javax.persistence.Transient;

import lombok.Getter;
import lombok.Setter;

@Entity
@Table(name = "AccessCode")
@Getter
@Setter
public class AccessCodeEntity extends ApplicationPersistenceEntity {

  private Timestamp creationTime;

  private Timestamp startTime;

  private Timestamp endTime;

  @OneToOne(cascade = CascadeType.DETACH, fetch = FetchType.EAGER)
  @JoinColumn(name = "idVisitor")
  private VisitorEntity visitor;

  @ManyToOne(cascade = CascadeType.DETACH, fetch = FetchType.EAGER)
  @JoinColumn(name = "idQueue")
  private QueueEntity queue;

  @Transient
  public Long getVisitorId() {

    if (this.visitor == null) {
      return null;
    }
    return this.visitor.getId();
  }

  public void setVisitorId(Long visitorId) {

    if (visitorId == null) {
      this.visitor = null;
    } else {
      VisitorEntity visitorEntity = new VisitorEntity();
      visitorEntity.setId(visitorId);
      this.visitor = visitorEntity;
    }
  }

  @Transient
  public Long getQueueId() {

    if (this.queue == null) {
      return null;
    }
    return this.queue.getId();
  }

  public void setQueueId(Long queueId) {

    if (queueId == null) {
      this.queue = null;
    } else {
      QueueEntity queueEntity = new QueueEntity();
      queueEntity.setId(queueId);
      this.queue = queueEntity;
    }
  }

}
