package com.devonfw.application.api.model;

import com.devonfw.module.basic.common.api.to.AbstractCto;

/**
 * Composite transport object of AccessCode
 */
public class AccessCodeCto extends AbstractCto {

  private static final long serialVersionUID = 1L;

  private AccessCodeEto accessCode;

  private VisitorEto visitor;

  private QueueEto queue;

  public AccessCodeEto getAccessCode() {

    return this.accessCode;
  }

  public void setAccessCode(AccessCodeEto accessCode) {

    this.accessCode = accessCode;
  }

  public VisitorEto getVisitor() {

    return this.visitor;
  }

  public void setVisitor(VisitorEto visitor) {

    this.visitor = visitor;
  }

  public QueueEto getQueue() {

    return this.queue;
  }

  public void setQueue(QueueEto queue) {

    this.queue = queue;
  }

}
