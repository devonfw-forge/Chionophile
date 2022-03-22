import { Controller } from '@nestjs/common';
import { Crud } from '@nestjsx/crud';
import { CrudType } from '@devon4node/common/serializer';
import { Visitor } from '../model/entities/visitor.entity';
import { VisitorCrudService } from '../services/visitor.crud.service';
import { ApiTags } from '@nestjs/swagger';

@Crud({
  model: {
    type: Visitor,
  },
})
@CrudType(Visitor)
@Controller('visitormanagement/v1/visitor')
@ApiTags('Visitor')
export class VisitorCrudController {
  constructor(public service: VisitorCrudService) {}
}
