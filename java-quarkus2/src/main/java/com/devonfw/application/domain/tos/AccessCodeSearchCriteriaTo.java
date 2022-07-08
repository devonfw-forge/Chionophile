package com.devonfw.application.domain.tos;

import java.sql.Timestamp;
import java.util.Date;

import com.devonfw.module.basic.common.api.query.StringSearchConfigTo;

import lombok.Getter;
import lombok.Setter;

/**
 * {@link SearchCriteriaTo} to find instances of
 * {@link com.devonfw.application.accesscodemanagement.common.api.AccessCode}s.
 */
@Getter
@Setter
public class AccessCodeSearchCriteriaTo extends AbstractSearchCriteriaTo {

  private static final long serialVersionUID = 1L;

  private String ticketNumber;

  private Date creationTime;

  private Date startTime;

  private Date endTime;

  private Long visitorId;

  private Long queueId;

  private StringSearchConfigTo ticketNumberOption;
}
