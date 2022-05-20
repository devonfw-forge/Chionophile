package com.devonfw.application.queuemanagement.logic.impl.usecase;

import java.util.Objects;

import javax.inject.Named;
import javax.validation.Valid;
import javax.transaction.Transactional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.devonfw.application.queuemanagement.dataaccess.api.QueueEntity;
import com.devonfw.application.queuemanagement.logic.api.to.QueueEto;
import com.devonfw.application.queuemanagement.logic.api.usecase.UcManageQueue;
import com.devonfw.application.queuemanagement.logic.base.usecase.AbstractQueueUc;

/**
 * Use case implementation for modifying and deleting Queues
 */
@Named
@Valid
@Transactional
public class UcManageQueueImpl extends AbstractQueueUc implements UcManageQueue {

  /** Logger instance. */
  private static final Logger LOG = LoggerFactory.getLogger(UcManageQueueImpl.class);

  @Override
  public boolean deleteQueue(long queueId) {

    QueueEntity queue = getQueueRepository().find(queueId);
    getQueueRepository().delete(queue);
    LOG.debug("The queue with id '{}' has been deleted.", queueId);
    return true;
  }

  @Override
  public QueueEto saveQueue(QueueEto queue) {

    Objects.requireNonNull(queue, "queue");

    QueueEntity queueEntity = getBeanMapper().map(queue);

    // initialize, validate queueEntity here if necessary
    QueueEntity resultEntity = getQueueRepository().save(queueEntity);
    LOG.debug("Queue with id '{}' has been created.", resultEntity.getId());
    return getBeanMapper().map(resultEntity);
  }
}
