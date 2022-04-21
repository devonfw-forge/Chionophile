import { Entity, Column, PrimaryGeneratedColumn, OneToOne, JoinColumn, ManyToOne } from 'typeorm';
import { Visitor } from '../../../visitor/model/entities/visitor.entity';
import { Queue } from '../../../queue/model/entities/queue.entity';
import { Transform } from 'class-transformer';

@Entity({ name: 'accesscode' })
export class AccessCode {
  @Transform(({ value }) => parseInt(value))
  @PrimaryGeneratedColumn()
  id: number;

  @Column('int', { name: 'modificationcounter', nullable: true })
  modificationCounter?: number = 1;

  @Column('timestamp', { name: 'creationtime' })
  creationTime?: Date = new Date();

  @Column('timestamp', { name: 'starttime' })
  startTime?: Date = new Date();

  @Column('timestamp', { name: 'endtime' })
  endTime?: Date = new Date();

  @Transform(({ value }) => parseInt(value))
  @Column('int', { name: 'idvisitor', nullable: true })
  visitorId?: number;

  @Transform(({ value }) => parseInt(value))
  @Column('int', { name: 'idqueue', nullable: true })
  queueId?: number;

  @OneToOne(() => Visitor)
  @JoinColumn({ name: 'id' })
  visitor: Visitor;

  @ManyToOne(() => Queue, queue => queue.accessCodes)
  @JoinColumn({ name: 'idqueue' })
  queue: Queue;
}
