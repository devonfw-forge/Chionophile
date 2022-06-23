package com.devonfw.application.domain.tos;

import com.devonfw.module.basic.common.api.query.StringSearchConfigTo;

import lombok.Setter;
import lombok.Getter;

/**
 * {@link SearchCriteriaTo} to find instances of
 * {@link com.devonfw.application.visitormanagement.common.api.Visitor}s.
 */
@Getter
@Setter
public class VisitorSearchCriteriaTo extends AbstractSearchCriteriaTo {

  private static final long serialVersionUID = 1L;

  private String username;

  private String name;

  private String phoneNumber;

  private String password;

  private Boolean acceptedCommercial;

  private Boolean acceptedTerms;

  private Boolean userType;

  private StringSearchConfigTo usernameOption;

  private StringSearchConfigTo nameOption;

  private StringSearchConfigTo phoneNumberOption;

  private StringSearchConfigTo passwordOption;
}
