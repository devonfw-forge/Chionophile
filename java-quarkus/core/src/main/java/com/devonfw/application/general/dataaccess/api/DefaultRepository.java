package com.devonfw.application.general.common.base.api;

import com.devonfw.application.general.common.api.GenericEntity;

public interface DefaultRepository<E extends GenericEntity<Long>> extends GenericRepository<E, Long> {

}