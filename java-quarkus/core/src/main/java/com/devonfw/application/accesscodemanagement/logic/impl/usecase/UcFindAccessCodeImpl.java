package com.devonfw.application.accesscodemanagement.logic.impl.usecase;

import java.util.ArrayList;
import java.util.List;

import javax.inject.Named;
import javax.validation.Valid;
import javax.transaction.Transactional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageImpl;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Pageable;

import com.devonfw.application.accesscodemanagement.dataaccess.api.AccessCodeEntity;
import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeCto;
import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeEto;
import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeSearchCriteriaTo;
import com.devonfw.application.accesscodemanagement.logic.api.usecase.UcFindAccessCode;
import com.devonfw.application.accesscodemanagement.logic.base.usecase.AbstractAccessCodeUc;
import com.devonfw.application.queuemanagement.logic.api.to.QueueEto;
import com.devonfw.application.visitormanagement.logic.api.to.VisitorEto;

/**
 * Use case implementation for searching, filtering and getting AccessCodes
 */
@Named
@Valid
@Transactional
public class UcFindAccessCodeImpl extends AbstractAccessCodeUc implements UcFindAccessCode {

  /** Logger instance. */
  private static final Logger LOG = LoggerFactory.getLogger(UcFindAccessCodeImpl.class);

  @Override
  public AccessCodeCto findAccessCodeCto(long id) {

    LOG.debug("Get AccessCodeCto with id {} from database.", id);
    AccessCodeEntity entity = getAccessCodeRepository().find(id);
    AccessCodeCto cto = new AccessCodeCto();
    AccessCodeEto acEto = getBeanMapper().map(entity);
    acEto.setTicketNumber(generateTicketCode(acEto.getId().intValue()));
    cto.setAccessCode(acEto);
    cto.setVisitor(getBeanMapper().map(entity.getVisitor()));
    cto.setQueue(getBeanMapper().map(entity.getQueue()));
    return cto;
  }

  @Override
  public Page<AccessCodeCto> findAccessCodeCtos(AccessCodeSearchCriteriaTo criteria) {

    Page<AccessCodeEntity> accesscodes = getAccessCodeRepository().findByCriteria(criteria);
    List<AccessCodeCto> ctos = new ArrayList<>();
    for (AccessCodeEntity entity : accesscodes.getContent()) {
      AccessCodeCto cto = new AccessCodeCto();
      AccessCodeEto acEto = getBeanMapper().map(entity);
      acEto.setTicketNumber(generateTicketCode(acEto.getId().intValue()));
      cto.setAccessCode(acEto);
      cto.setVisitor(getBeanMapper().map(entity.getVisitor()));
      cto.setQueue(getBeanMapper().map(entity.getQueue()));
      ctos.add(cto);
    }
    Pageable pagResultTo = PageRequest.of(criteria.getPageable().getPageNumber(), criteria.getPageable().getPageSize());

    return new PageImpl<>(ctos, pagResultTo, accesscodes.getTotalElements());
  }

  @Override
  public Page<AccessCodeEto> findAccessCodeEtos(AccessCodeSearchCriteriaTo criteria) {

    Page<AccessCodeEntity> accessCodes = getAccessCodeRepository().findByCriteria(criteria);

    return mapPaginatedAccessCodeEntityList(accessCodes);
  }

  /**
   * Generates a new ticked code using the ticket digit of the last codeaccess
   * created.
   *
   * @param lastTicketDigit the int of the last codeaccess created.
   * @return the String with the new ticket code (example: 'Q005').
   */
  public static String generateTicketCode(int lastTicketDigit) {

    int newTicketDigit = lastTicketDigit;
    String newTicketCode = "";
    if (newTicketDigit == 1000) {
      newTicketCode = "Q000";
    } else {
      StringBuilder stringBuilder = new StringBuilder();
      stringBuilder.append(newTicketDigit);
      while (stringBuilder.length() < 3) {
        stringBuilder.insert(0, "0");
      }
      stringBuilder.insert(0, "Q");
      newTicketCode = stringBuilder.toString();
    }
    return newTicketCode;
  }

}
