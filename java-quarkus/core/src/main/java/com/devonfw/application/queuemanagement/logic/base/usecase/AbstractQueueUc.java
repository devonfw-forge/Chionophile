package com.devonfw.application.queuemanagement.logic.base.usecase;

import javax.inject.Inject;

import com.devonfw.application.general.logic.base.AbstractUc;
import com.devonfw.application.queuemanagement.dataaccess.api.repo.QueueRepository;

/**
 * Abstract use case for Queues, which provides access to the commonly necessary data access objects.
 */
public abstract class AbstractQueueUc extends AbstractUc {

  /** @see #getQueueRepository() */
  @Inject
  private QueueRepository queueRepository;

  /**
   * @return the {@link QueueRepository} instance.
   */
  public QueueRepository getQueueRepository() {

    return this.queueRepository;
  }

}
