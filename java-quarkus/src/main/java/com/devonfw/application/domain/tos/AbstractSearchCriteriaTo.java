package com.devonfw.application.domain.tos;

import org.springframework.data.domain.Pageable;

import com.devonfw.module.basic.common.api.to.AbstractTo;

import lombok.Getter;
import lombok.Setter;

/**
 * Abstract {@link AbstractTo TO} for search criteria.
 */
@Getter
@Setter
public abstract class AbstractSearchCriteriaTo extends AbstractTo {

  private static final long serialVersionUID = 1L;

  private Pageable pageable;
}
