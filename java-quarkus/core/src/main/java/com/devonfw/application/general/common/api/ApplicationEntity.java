package com.devonfw.application.general.common.api;

import com.devonfw.application.general.common.api.GenericEntity;

/**
 * This is the abstract interface for a {@link GenericEntity}. We are using
 * {@link Long} for all {@link #getId() primary
 * keys}.
 */
public abstract interface ApplicationEntity extends GenericEntity<Long> {

}
