package com.devonfw.application.accesscodemanagement.logic.impl.usecase;

import java.sql.Timestamp;
import java.time.Instant;
import java.util.Objects;

import javax.inject.Inject;
import javax.inject.Named;
import javax.validation.Valid;
import javax.transaction.Transactional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Pageable;

import com.devonfw.application.accesscodemanagement.dataaccess.api.AccessCodeEntity;
import com.devonfw.application.accesscodemanagement.logic.api.Accesscodemanagement;
import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeEto;
import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeSearchCriteriaTo;
import com.devonfw.application.accesscodemanagement.logic.api.usecase.UcManageAccessCode;
import com.devonfw.application.accesscodemanagement.logic.base.usecase.AbstractAccessCodeUc;
import com.devonfw.application.queuemanagement.logic.api.Queuemanagement;
import com.devonfw.application.queuemanagement.logic.impl.usecase.UcManageQueueImpl;

@Named
@Valid
@Transactional
public class UcManageAccessCodeImpl extends AbstractAccessCodeUc implements UcManageAccessCode {

  @Inject
  private Queuemanagement queuemanagement;

  @Inject
  private Accesscodemanagement accesscodemanagement;

  /** Logger instance. */
  private static final Logger LOG = LoggerFactory.getLogger(UcManageQueueImpl.class);

  @Override
  public void deleteAccessCode(long accessCodeId) {

    // then we delete the accesscode
    getAccessCodeRepository().deleteById(accessCodeId);
    LOG.debug("The accesscode with id '{}' has been deleted.", accessCodeId);

  }

  @Override
  public AccessCodeEto saveAccessCode(AccessCodeEto accessCodeEto) {

    // make sure the object is not null
    Objects.requireNonNull(accessCodeEto, "UcManageAccessImpl accessCode null");

    AccessCodeEntity accessCodeEntity = getBeanMapper().map(accessCodeEto);

    long queueEntityId = accessCodeEntity.getQueueId();

    AccessCodeSearchCriteriaTo accessCodeSearchCriteriaTo = new AccessCodeSearchCriteriaTo();
    accessCodeSearchCriteriaTo.setQueueId(queueEntityId);
    Pageable pageable = PageRequest.of(0, 1000);
    accessCodeSearchCriteriaTo.setPageable(pageable);

    /**
     * Calling the parent with the method getAccesscodemanagement() we use the
     * method findAccessCodeEtos() that will
     * call the implementation of the method inside (UcFindAccessCodeImpl) through
     * the interface. This allows us to use
     * the {@link UcFindAccessCodeImpl}.
     */
    // List<AccessCodeEto> accessCodeEtosInQueue =
    // getAccesscodemanagement().findAccessCodeEtos(accessCodeSearchCriteriaTo)
    // .getContent();

    // set the creation time, startTime and endTime
    accessCodeEntity.setCreationTime(Timestamp.from(Instant.now()));
    accessCodeEntity.setStartTime(null);
    accessCodeEntity.setEndTime(null);

    // save the AccessCode
    AccessCodeEntity accessCodeEntitySaved = getAccessCodeRepository().save(accessCodeEntity);
    LOG.debug("The accesscode with id '{}' has been saved.", accessCodeEntitySaved.getId());
    return getBeanMapper().map(accessCodeEntitySaved);
  }

  public Queuemanagement getQueuemanagement() {

    return this.queuemanagement;
  }

  public Accesscodemanagement getAccesscodemanagement() {

    return this.accesscodemanagement;
  }

}