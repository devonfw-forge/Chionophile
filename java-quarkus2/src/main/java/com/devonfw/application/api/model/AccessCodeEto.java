package com.devonfw.application.api.model;

import java.sql.Timestamp;

import com.devonfw.module.basic.common.api.to.AbstractEto;

import lombok.Getter;
import lombok.Setter;

/**
 * Entity transport object of AccessCode
 */
@Getter
@Setter
public class AccessCodeEto extends AbstractEto {

  private static final long serialVersionUID = 1L;

  private String ticketNumber;

  private Timestamp creationTime;

  private Timestamp startTime;

  private Timestamp endTime;

  private Long visitorId;

  private Long queueId;

  @Override
  public int hashCode() {

    final int prime = 31;
    int result = super.hashCode();
    result = prime * result + ((this.creationTime == null) ? 0 : this.creationTime.hashCode());
    result = prime * result + ((this.startTime == null) ? 0 : this.startTime.hashCode());
    result = prime * result + ((this.endTime == null) ? 0 : this.endTime.hashCode());

    result = prime * result + ((this.visitorId == null) ? 0 : this.visitorId.hashCode());

    result = prime * result + ((this.queueId == null) ? 0 : this.queueId.hashCode());
    return result;
  }

  @Override
  public boolean equals(Object obj) {

    if (this == obj) {
      return true;
    }
    if (obj == null) {
      return false;
    }
    // class check will be done by super type EntityTo!
    if (!super.equals(obj)) {
      return false;
    }
    AccessCodeEto other = (AccessCodeEto) obj;
    if (this.creationTime == null) {
      if (other.creationTime != null) {
        return false;
      }
    } else if (!this.creationTime.equals(other.creationTime)) {
      return false;
    }
    if (this.startTime == null) {
      if (other.startTime != null) {
        return false;
      }
    } else if (!this.startTime.equals(other.startTime)) {
      return false;
    }
    if (this.endTime == null) {
      if (other.endTime != null) {
        return false;
      }
    } else if (!this.endTime.equals(other.endTime)) {
      return false;
    }

    if (this.visitorId == null) {
      if (other.visitorId != null) {
        return false;
      }
    } else if (!this.visitorId.equals(other.visitorId)) {
      return false;
    }

    if (this.queueId == null) {
      if (other.queueId != null) {
        return false;
      }
    } else if (!this.queueId.equals(other.queueId)) {
      return false;
    }
    return true;
  }
}
