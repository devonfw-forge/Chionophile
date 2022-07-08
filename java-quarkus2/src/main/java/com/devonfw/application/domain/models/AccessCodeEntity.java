package com.devonfw.application.domain.models;

import java.util.Date;

import javax.persistence.*;

import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

@Entity
@Table(name = "AccessCode")
@Getter
@Setter
@ToString
public class AccessCodeEntity extends ApplicationPersistenceEntity {

  @Temporal(TemporalType.TIMESTAMP)
  private Date creationTime;

  @Temporal(TemporalType.TIMESTAMP)
  private Date startTime;

  @Temporal(TemporalType.TIMESTAMP)
  private Date endTime;

  // , fetch = FetchType.EAGER = eager is evil and you should almost never use it (perf issues, google Hibernate N+1)
  @OneToOne(cascade = CascadeType.DETACH)
  @JoinColumn(name = "idVisitor")
  private VisitorEntity visitor;

  @ManyToOne(cascade = CascadeType.DETACH)
  @JoinColumn(name = "idQueue")
  private QueueEntity queue;
}
