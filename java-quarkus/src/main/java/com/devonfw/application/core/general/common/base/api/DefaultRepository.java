package com.devonfw.application.general.common.base.api;

import com.devonfw.module.basic.common.api.entity.PersistenceEntity;

public interface DefaultRepository<E extends PersistenceEntity<Long>> extends GenericRepository<E, Long> {

}