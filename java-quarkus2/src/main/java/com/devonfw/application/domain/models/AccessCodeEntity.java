package com.devonfw.application.domain.models;

import java.util.Date;

import javax.persistence.CascadeType;
import javax.persistence.Entity;
import javax.persistence.FetchType;
import javax.persistence.JoinColumn;
import javax.persistence.ManyToOne;
import javax.persistence.NamedAttributeNode;
import javax.persistence.NamedEntityGraph;
import javax.persistence.OneToOne;
import javax.persistence.Table;
import javax.persistence.Transient;

import lombok.Getter;
import lombok.Setter;

@Entity
@Table(name = "AccessCode")
@Getter
@Setter
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
