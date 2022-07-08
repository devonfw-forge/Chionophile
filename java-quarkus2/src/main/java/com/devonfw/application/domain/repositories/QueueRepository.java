package com.devonfw.application.domain.repositories;

import org.springframework.data.domain.Page;
import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.QueueEntity;
import com.devonfw.application.domain.tos.QueueSearchCriteriaTo;

/**
 * {@link DefaultRepository} for {@link QueueEntity}
 */
public interface QueueRepository extends JpaRepository<QueueEntity, Long>, QueueRepositoryFragment {

}