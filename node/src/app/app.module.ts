import { Module } from '@nestjs/common';
import { CoreModule } from './core/core.module';
import { AccessCodeModule } from './accesscode/accessCode.module';
import { QueueModule } from './queue/queue.module';
import { VisitorModule } from './visitor/visitor.module';

@Module({
  imports: [CoreModule, AccessCodeModule, QueueModule, VisitorModule],
  controllers: [],
  providers: [],
})
export class AppModule {}
