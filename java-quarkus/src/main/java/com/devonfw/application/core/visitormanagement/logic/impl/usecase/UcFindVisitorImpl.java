package com.devonfw.application.visitormanagement.logic.impl.usecase;

import java.util.Optional;

import javax.inject.Named;
import javax.validation.Valid;
import javax.transaction.Transactional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.data.domain.Page;

import com.devonfw.application.visitormanagement.dataaccess.api.VisitorEntity;
import com.devonfw.application.visitormanagement.logic.api.to.VisitorEto;
import com.devonfw.application.visitormanagement.logic.api.to.VisitorSearchCriteriaTo;
import com.devonfw.application.visitormanagement.logic.api.usecase.UcFindVisitor;
import com.devonfw.application.visitormanagement.logic.base.usecase.AbstractVisitorUc;

/**
 * Use case implementation for searching, filtering and getting Visitors
 */
@Named
@Valid
@Transactional
public class UcFindVisitorImpl extends AbstractVisitorUc implements UcFindVisitor {

  /** Logger instance. */
  private static final Logger LOG = LoggerFactory.getLogger(UcFindVisitorImpl.class);

  @Override
  public VisitorEto findVisitor(long id) {

    LOG.debug("Get Visitor with id {} from database.", id);
    Optional<VisitorEntity> foundEntity = getVisitorRepository().findById(id);
    if (foundEntity.isPresent())
      return getBeanMapper().map(foundEntity.get());
    else
      return null;
  }

  @Override
  public Page<VisitorEto> findVisitors(VisitorSearchCriteriaTo criteria) {

    Page<VisitorEntity> visitors = getVisitorRepository().findByCriteria(criteria);
    return mapPaginatedVisitorEntityList(visitors);
  }

}
