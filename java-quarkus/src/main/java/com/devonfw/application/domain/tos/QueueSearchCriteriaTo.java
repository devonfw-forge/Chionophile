package com.devonfw.application.domain.tos;

import java.sql.Timestamp;

import com.devonfw.module.basic.common.api.query.StringSearchConfigTo;

import lombok.Getter;
import lombok.Setter;

/**
 * {@link SearchCriteriaTo} to find instances of
 * {@link com.devonfw.application.queuemanagement.common.api.Queue}s.
 */
@Getter
@Setter
public class QueueSearchCriteriaTo extends AbstractSearchCriteriaTo {

  private static final long serialVersionUID = 1L;

  private String name;

  private String logo;

  private String currentNumber;

  private Timestamp attentionTime;

  private Timestamp minAttentionTime;

  private Boolean active;

  private StringSearchConfigTo nameOption;

  private StringSearchConfigTo logoOption;

  private StringSearchConfigTo currentNumberOption;
}
