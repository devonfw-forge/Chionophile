package main.java.com.devonfw.application.accesscodemanagement.dataaccess.api;

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

import com.devonfw.application.jtqj.accesscodemanagement.common.api.AccessCode;
import com.devonfw.application.jtqj.general.dataaccess.api.ApplicationPersistenceEntity;
import com.devonfw.application.jtqj.queuemanagement.dataaccess.api.QueueEntity;
import com.devonfw.application.jtqj.visitormanagement.dataaccess.api.VisitorEntity;

@Entity
@Table(name = "AccessCode")
public class AccessCodeEntity extends ApplicationPersistenceEntity implements AccessCode {

  @Temporal(TemporalType.TIMESTAMP)
  private Timestamp creationTime;

  @Temporal(TemporalType.TIMESTAMP)
  private Timestamp startTime;

  @Temporal(TemporalType.TIMESTAMP)
  private Timestamp endTime;

  private VisitorEntity visitor;

  private QueueEntity queue;

  private static final long serialVersionUID = 1L;

  /**
   * @return the creationTime
   */
  @Override
  public Timestamp getCreationTime() {

    return this.creationTime;
  }

  /**
   * @param creationTime the creationTime to set
   */
  @Override
  public void setCreationTime(Timestamp creationTime) {

    this.creationTime = creationTime;
  }

  /**
   * @return the startTime
   */
  @Override
  public Timestamp getStartTime() {

    return this.startTime;
  }

  /**
   * @param startTime the startTime to set
   */
  @Override
  public void setStartTime(Timestamp startTime) {

    this.startTime = startTime;
  }

  /**
   * @return the endTime
   */
  @Override
  public Timestamp getEndTime() {

    return this.endTime;
  }

  /**
   * @param endTime the endTime to set
   */
  @Override
  public void setEndTime(Timestamp endTime) {

    this.endTime = endTime;
  }

  /**
   * @return the visitor
   */
  @OneToOne(cascade = CascadeType.DETACH, fetch = FetchType.EAGER)
  @JoinColumn(name = "idVisitor")
  public VisitorEntity getVisitor() {

    return this.visitor;
  }

  /**
   * @param visitor the visitor to set
   */
  public void setVisitor(VisitorEntity visitor) {

    this.visitor = visitor;
  }

  /**
   * @return the queue
   */
  @ManyToOne(cascade = CascadeType.DETACH, fetch = FetchType.EAGER)
  @JoinColumn(name = "idQueue")
  public QueueEntity getQueue() {

    return this.queue;
  }

  /**
   * @param queue the queue to set
   */
  public void setQueue(QueueEntity queue) {

    this.queue = queue;
  }

  @Override
  @Transient
  public Long getVisitorId() {

    if (this.visitor == null) {
      return null;
    }
    return this.visitor.getId();
  }

  @Override
  public void setVisitorId(Long visitorId) {

    if (visitorId == null) {
      this.visitor = null;
    } else {
      VisitorEntity visitorEntity = new VisitorEntity();
      visitorEntity.setId(visitorId);
      this.visitor = visitorEntity;
    }
  }

  @Override
  @Transient
  public Long getQueueId() {

    if (this.queue == null) {
      return null;
    }
    return this.queue.getId();
  }

  @Override
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
