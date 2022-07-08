package com.devonfw.application.domain.repositories;

import org.springframework.data.domain.Page;
import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.AccessCodeEntity;
import com.devonfw.application.domain.tos.AccessCodeSearchCriteriaTo;

public interface AccessCodeRepository extends JpaRepository<AccessCodeEntity, Long>, AccessCodeRepositoryFragment {

}