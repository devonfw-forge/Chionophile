package com.devonfw.application.domain.repositories;

import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.AccessCodeEntity;

public interface AccessCodeRepository extends JpaRepository<AccessCodeEntity, Long>, AccessCodeRepositoryFragment {

}