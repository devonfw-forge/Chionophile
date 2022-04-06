import { Entity, Column, PrimaryGeneratedColumn, OneToOne, JoinColumn, ManyToOne } from 'typeorm';
import { CrudValidationGroups } from '@nestjsx/crud';
import { IsDefined, IsOptional } from 'class-validator';
import { Visitor } from '../../../visitor/model/entities/visitor.entity';
import { Queue } from '../../../queue/model/entities/queue.entity';

@Entity({ name: 'accesscode' })
export class AccessCode {
  @PrimaryGeneratedColumn()
  id: number;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('int', { name: 'modificationcounter', nullable: true })
  modificationCounter?: number;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', {name: 'creationtime',  nullable: true })
  creationTime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', {name: 'starttime', nullable: true })
  startTime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', {name: 'endtime', nullable: true })
  endTime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('int', {name: 'idvisitor', nullable: true })
  visitorId?: number;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('int', {name: 'idqueue', nullable: true })
  queueId?: number;

  @OneToOne(() => Visitor)
  @JoinColumn({ name: 'id' })
  visitor: Visitor;

  @ManyToOne(() => Queue, (queue) => queue.accessCodes)
  @JoinColumn({ name: 'idqueue' })
  queue: Queue;
}
