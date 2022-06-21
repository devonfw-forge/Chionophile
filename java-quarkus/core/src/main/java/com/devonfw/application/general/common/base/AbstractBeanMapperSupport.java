package com.devonfw.application.general.common.base;

import com.devonfw.application.general.common.api.mapping.JTQMapper;

import javax.inject.Inject;

/**
 * This abstract class provides {@link #getBeanMapper() access} to the {@link
 * BeanMapper}.
 */
public abstract class AbstractBeanMapperSupport {

  @Inject
  private JTQMapper beanMapper;

  /**
   * @param beanMapper is the {@link BeanMapper} to {@link Inject}
   */
  // @Inject
  // public void setBeanMapper(BeanMapper beanMapper) {

  // this.beanMapper = beanMapper;
  // }

  /**
   * @return the {@link BeanMapper} instance.
   */
  protected JTQMapper getBeanMapper() {

    return this.beanMapper;
  }
}
