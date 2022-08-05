package com.devonfw.application.domain.tos;

import java.sql.Timestamp;

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

  private Timestamp creationTime;

  private Timestamp startTime;

  private Timestamp endTime;

  private Long visitorId;

  private Long queueId;

  private StringSearchConfigTo ticketNumberOption;
}
