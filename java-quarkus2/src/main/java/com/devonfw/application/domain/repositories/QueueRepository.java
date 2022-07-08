package com.devonfw.application.domain.repositories;

import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.QueueEntity;

/**
 * {@link DefaultRepository} for {@link QueueEntity}
 */
public interface QueueRepository extends JpaRepository<QueueEntity, Long>, QueueRepositoryFragment {

}